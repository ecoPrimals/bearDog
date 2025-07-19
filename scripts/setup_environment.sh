#!/bin/bash

# BearDog Environment Setup Script
# This script helps set up proper environment variables and secrets for BearDog deployment

set -e

echo "🐻 BearDog Environment Setup Script"
echo "===================================="

# Default values
ENVIRONMENT=${BEARDOG_ENVIRONMENT:-development}
CONFIG_DIR=${BEARDOG_CONFIG_DIR:-/etc/beardog}
SECRETS_DIR=${BEARDOG_SECRETS_DIR:-/etc/beardog/secrets}
LOG_DIR=${BEARDOG_LOG_DIR:-/var/log/beardog}
DATA_DIR=${BEARDOG_DATA_DIR:-/var/lib/beardog}

echo "Environment: $ENVIRONMENT"
echo "Config Directory: $CONFIG_DIR"
echo "Secrets Directory: $SECRETS_DIR"
echo "Log Directory: $LOG_DIR"
echo "Data Directory: $DATA_DIR"

# Function to generate a secure random string
generate_secret() {
    local length=${1:-32}
    openssl rand -base64 $length | tr -d "=+/" | cut -c1-$length
}

# Function to create directory with proper permissions
create_secure_dir() {
    local dir=$1
    local permissions=${2:-755}
    
    if [ ! -d "$dir" ]; then
        echo "Creating directory: $dir"
        sudo mkdir -p "$dir"
        sudo chmod $permissions "$dir"
        
        if [ "$ENVIRONMENT" = "production" ]; then
            sudo chown beardog:beardog "$dir" 2>/dev/null || true
        fi
    fi
}

# Function to create a secret file
create_secret_file() {
    local secret_name=$1
    local secret_value=$2
    local secret_file="$SECRETS_DIR/$secret_name.secret"
    
    echo "Creating secret file: $secret_file"
    echo "$secret_value" | sudo tee "$secret_file" > /dev/null
    sudo chmod 600 "$secret_file"
    
    if [ "$ENVIRONMENT" = "production" ]; then
        sudo chown beardog:beardog "$secret_file" 2>/dev/null || true
    fi
}

# Function to set environment variable
set_env_var() {
    local var_name=$1
    local var_value=$2
    local env_file="/etc/environment"
    
    # Check if the variable is already set
    if [ -z "${!var_name}" ]; then
        echo "Setting environment variable: $var_name"
        
        # Add to /etc/environment for system-wide access
        if [ "$ENVIRONMENT" = "production" ]; then
            echo "$var_name=\"$var_value\"" | sudo tee -a "$env_file" > /dev/null
        else
            export "$var_name"="$var_value"
            echo "export $var_name=\"$var_value\"" >> ~/.bashrc
        fi
    else
        echo "Environment variable $var_name already set"
    fi
}

# Create necessary directories
echo ""
echo "📁 Creating directories..."
create_secure_dir "$CONFIG_DIR" 755
create_secure_dir "$SECRETS_DIR" 700
create_secure_dir "$LOG_DIR" 755
create_secure_dir "$DATA_DIR" 755

# Generate secrets if they don't exist
echo ""
echo "🔐 Setting up secrets..."

# JWT Secret
if [ -z "$BEARDOG_JWT_SECRET" ] && [ ! -f "$SECRETS_DIR/jwt_secret.secret" ]; then
    JWT_SECRET=$(generate_secret 64)
    create_secret_file "jwt_secret" "$JWT_SECRET"
    set_env_var "BEARDOG_JWT_SECRET" "$JWT_SECRET"
fi

# Encryption Key
if [ -z "$BEARDOG_ENCRYPTION_KEY" ] && [ ! -f "$SECRETS_DIR/encryption_key.secret" ]; then
    ENCRYPTION_KEY=$(generate_secret 32)
    create_secret_file "encryption_key" "$ENCRYPTION_KEY"
    set_env_var "BEARDOG_ENCRYPTION_KEY" "$ENCRYPTION_KEY"
fi

# API Key
if [ -z "$BEARDOG_API_KEY" ] && [ ! -f "$SECRETS_DIR/api_key.secret" ]; then
    API_KEY=$(generate_secret 32)
    create_secret_file "api_key" "$API_KEY"
    set_env_var "BEARDOG_API_KEY" "$API_KEY"
fi

# Database Password (if using PostgreSQL)
if [ -z "$BEARDOG_DATABASE_PASSWORD" ] && [ ! -f "$SECRETS_DIR/database_password.secret" ]; then
    if [ "$ENVIRONMENT" = "production" ]; then
        DB_PASSWORD=$(generate_secret 32)
        create_secret_file "database_password" "$DB_PASSWORD"
        set_env_var "BEARDOG_DATABASE_PASSWORD" "$DB_PASSWORD"
    fi
fi

# NestGate Secret
if [ -z "$BEARDOG_NESTGATE_SECRET" ] && [ ! -f "$SECRETS_DIR/nestgate_secret.secret" ]; then
    if [ "$ENVIRONMENT" = "development" ]; then
        NESTGATE_SECRET="dev-nestgate-secret"
    else
        NESTGATE_SECRET=$(generate_secret 32)
    fi
    create_secret_file "nestgate_secret" "$NESTGATE_SECRET"
    set_env_var "BEARDOG_NESTGATE_SECRET" "$NESTGATE_SECRET"
fi

# SongBird Secret
if [ -z "$BEARDOG_SONGBIRD_SECRET" ] && [ ! -f "$SECRETS_DIR/songbird_secret.secret" ]; then
    if [ "$ENVIRONMENT" = "development" ]; then
        SONGBIRD_SECRET="dev-songbird-secret"
    else
        SONGBIRD_SECRET=$(generate_secret 32)
    fi
    create_secret_file "songbird_secret" "$SONGBIRD_SECRET"
    set_env_var "BEARDOG_SONGBIRD_SECRET" "$SONGBIRD_SECRET"
fi

# Admin Password
if [ -z "$BEARDOG_ADMIN_PASSWORD" ] && [ ! -f "$SECRETS_DIR/admin_password.secret" ]; then
    if [ "$ENVIRONMENT" = "development" ]; then
        ADMIN_PASSWORD="admin123"
    else
        ADMIN_PASSWORD=$(generate_secret 24)
    fi
    create_secret_file "admin_password" "$ADMIN_PASSWORD"
    set_env_var "BEARDOG_ADMIN_PASSWORD" "$ADMIN_PASSWORD"
fi

# HSM PIN (if HSM is enabled)
if [ "$ENVIRONMENT" = "production" ] && [ -z "$BEARDOG_HSM_PIN" ] && [ ! -f "$SECRETS_DIR/hsm_pin.secret" ]; then
    read -p "Enter HSM PIN (leave empty to generate): " HSM_PIN
    if [ -z "$HSM_PIN" ]; then
        HSM_PIN=$(generate_secret 8)
    fi
    create_secret_file "hsm_pin" "$HSM_PIN"
    set_env_var "BEARDOG_HSM_PIN" "$HSM_PIN"
fi

# Set up environment-specific configuration
echo ""
echo "⚙️  Setting up environment configuration..."

# Database URL
if [ "$ENVIRONMENT" = "development" ]; then
    set_env_var "BEARDOG_DATABASE_URL" "sqlite://./dev-beardog.db"
else
    if [ -z "$BEARDOG_DATABASE_URL" ]; then
        read -p "Enter database URL (e.g., postgresql://user:pass@host:5432/db): " DATABASE_URL
        set_env_var "BEARDOG_DATABASE_URL" "$DATABASE_URL"
    fi
fi

# API Bind Address
if [ "$ENVIRONMENT" = "development" ]; then
    set_env_var "BEARDOG_API_BIND_ADDRESS" "127.0.0.1:8080"
else
    set_env_var "BEARDOG_API_BIND_ADDRESS" "0.0.0.0:8443"
fi

# Log Level
if [ "$ENVIRONMENT" = "development" ]; then
    set_env_var "BEARDOG_LOG_LEVEL" "DEBUG"
else
    set_env_var "BEARDOG_LOG_LEVEL" "INFO"
fi

# Environment
set_env_var "BEARDOG_ENVIRONMENT" "$ENVIRONMENT"

# Debug Mode
if [ "$ENVIRONMENT" = "development" ]; then
    set_env_var "BEARDOG_DEBUG" "true"
else
    set_env_var "BEARDOG_DEBUG" "false"
fi

# Setup configuration files
echo ""
echo "📄 Setting up configuration files..."

# Copy appropriate configuration file
if [ "$ENVIRONMENT" = "production" ]; then
    if [ -f "production-config.toml" ]; then
        sudo cp production-config.toml "$CONFIG_DIR/beardog.toml"
        echo "Copied production configuration to $CONFIG_DIR/beardog.toml"
    fi
else
    if [ -f "development-config.toml" ]; then
        sudo cp development-config.toml "$CONFIG_DIR/beardog.toml"
        echo "Copied development configuration to $CONFIG_DIR/beardog.toml"
    fi
fi

# Set proper permissions
if [ -f "$CONFIG_DIR/beardog.toml" ]; then
    sudo chmod 644 "$CONFIG_DIR/beardog.toml"
    if [ "$ENVIRONMENT" = "production" ]; then
        sudo chown beardog:beardog "$CONFIG_DIR/beardog.toml" 2>/dev/null || true
    fi
fi

# Create systemd service file for production
if [ "$ENVIRONMENT" = "production" ]; then
    echo ""
    echo "📦 Creating systemd service..."
    
    sudo tee /etc/systemd/system/beardog.service > /dev/null <<EOF
[Unit]
Description=BearDog Security Manager
After=network.target postgresql.service
Wants=postgresql.service

[Service]
Type=simple
User=beardog
Group=beardog
WorkingDirectory=$DATA_DIR
ExecStart=/usr/local/bin/beardog --config $CONFIG_DIR/beardog.toml
Restart=always
RestartSec=10
Environment=BEARDOG_CONFIG_FILE=$CONFIG_DIR/beardog.toml
Environment=BEARDOG_SECRETS_DIR=$SECRETS_DIR
Environment=BEARDOG_LOG_DIR=$LOG_DIR
Environment=BEARDOG_DATA_DIR=$DATA_DIR

# Security settings
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ReadWritePaths=$DATA_DIR $LOG_DIR $SECRETS_DIR
ProtectHome=true
ProtectKernelTunables=true
ProtectKernelModules=true
ProtectControlGroups=true

[Install]
WantedBy=multi-user.target
EOF

    echo "Created systemd service file: /etc/systemd/system/beardog.service"
    sudo systemctl daemon-reload
    sudo systemctl enable beardog
fi

# Create log rotation configuration
echo ""
echo "📋 Setting up log rotation..."

sudo tee /etc/logrotate.d/beardog > /dev/null <<EOF
$LOG_DIR/*.log {
    daily
    rotate 30
    compress
    delaycompress
    missingok
    notifempty
    create 644 beardog beardog
    postrotate
        systemctl reload beardog || true
    endscript
}
EOF

echo "Created log rotation configuration: /etc/logrotate.d/beardog"

# Set up backup scripts for production
if [ "$ENVIRONMENT" = "production" ]; then
    echo ""
    echo "💾 Setting up backup scripts..."
    
    # Create backup directory
    BACKUP_DIR="/var/backups/beardog"
    create_secure_dir "$BACKUP_DIR" 700
    
    # Create database backup script
    sudo tee "$CONFIG_DIR/backup-database.sh" > /dev/null <<EOF
#!/bin/bash
# BearDog Database Backup Script
set -e

BACKUP_DIR="$BACKUP_DIR"
TIMESTAMP=\$(date +%Y%m%d_%H%M%S)
BACKUP_FILE="\$BACKUP_DIR/beardog_backup_\$TIMESTAMP.sql"

# Create database backup
pg_dump "\$BEARDOG_DATABASE_URL" > "\$BACKUP_FILE"

# Compress the backup
gzip "\$BACKUP_FILE"

# Keep only last 7 days of backups
find "\$BACKUP_DIR" -name "beardog_backup_*.sql.gz" -mtime +7 -delete

echo "Database backup completed: \$BACKUP_FILE.gz"
EOF

    sudo chmod 755 "$CONFIG_DIR/backup-database.sh"
    
    # Create cron job for daily backups
    echo "0 2 * * * root $CONFIG_DIR/backup-database.sh" | sudo tee -a /etc/crontab > /dev/null
    
    echo "Created backup script: $CONFIG_DIR/backup-database.sh"
fi

# Final security check
echo ""
echo "🔒 Final security checks..."

# Check file permissions
echo "Checking file permissions..."
find "$SECRETS_DIR" -type f -exec ls -la {} \; | grep -v "600" && echo "WARNING: Some secret files have incorrect permissions!"

# Check for empty secrets
echo "Checking for empty secrets..."
find "$SECRETS_DIR" -name "*.secret" -size 0 && echo "WARNING: Some secret files are empty!"

# Display summary
echo ""
echo "✅ Environment setup complete!"
echo ""
echo "Summary:"
echo "- Environment: $ENVIRONMENT"
echo "- Config file: $CONFIG_DIR/beardog.toml"
echo "- Secrets directory: $SECRETS_DIR"
echo "- Log directory: $LOG_DIR"
echo "- Data directory: $DATA_DIR"
echo ""

if [ "$ENVIRONMENT" = "production" ]; then
    echo "Production setup complete. You can now:"
    echo "1. Start the service: sudo systemctl start beardog"
    echo "2. Check status: sudo systemctl status beardog"
    echo "3. View logs: sudo journalctl -u beardog -f"
    echo ""
    echo "⚠️  Important: Review and update the configuration file before starting the service!"
    echo "⚠️  Ensure all secrets are properly configured and secure!"
else
    echo "Development setup complete. You can now:"
    echo "1. Source the environment: source ~/.bashrc"
    echo "2. Run BearDog: cargo run"
    echo "3. View logs: tail -f $LOG_DIR/beardog.log"
    echo ""
    echo "💡 Development secrets have been set to simple values for convenience."
fi

echo ""
echo "🔐 Secret Management:"
echo "- Secrets are stored in: $SECRETS_DIR"
echo "- Environment variables are set for: JWT, Encryption, API keys"
echo "- Configuration supports both environment variables and file-based secrets"
echo ""
echo "📚 Next steps:"
echo "1. Review the configuration file"
echo "2. Update any service endpoints"
echo "3. Test the configuration"
echo "4. Set up monitoring and alerting"
echo ""
echo "🐻 BearDog is ready to protect your systems!" 