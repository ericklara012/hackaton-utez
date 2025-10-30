#!/bin/bash

# Script de despliegue automatizado para AgroCoin Smart Contract
# Uso: ./deploy.sh [testnet|mainnet]

set -e

NETWORK=${1:-testnet}
CONTRACT_NAME="agrocoin-contract"

echo "🚀 Desplegando AgroCoin Smart Contract en $NETWORK..."

# Verificar que Soroban CLI esté instalado
if ! command -v soroban &> /dev/null; then
    echo "❌ Soroban CLI no está instalado. Instálalo con:"
    echo "cargo install --locked soroban-cli"
    exit 1
fi

# Verificar que el target wasm32 esté instalado
if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
    echo "📦 Instalando target wasm32-unknown-unknown..."
    rustup target add wasm32-unknown-unknown
fi

# Compilar el contrato
echo "🔨 Compilando contrato..."
cargo build --target wasm32-unknown-unknown --release

# Optimizar WASM
echo "⚡ Optimizando WASM..."
soroban contract optimize --wasm target/wasm32-unknown-unknown/release/agrocoin_contract.wasm

# Configurar red
if [ "$NETWORK" = "testnet" ]; then
    echo "🌐 Configurando red testnet..."
    soroban network add \
        --global testnet \
        --rpc-url https://soroban-testnet.stellar.org:443 \
        --network-passphrase "Test SDF Network ; September 2015" 2>/dev/null || true

    # Verificar si la identidad existe
    if ! soroban keys show deployer --network testnet &> /dev/null; then
        echo "🔑 Generando nueva identidad para deployment..."
        soroban keys generate --global deployer --network testnet

        echo "💰 Obteniendo fondos de testnet..."
        soroban keys fund deployer --network testnet
    fi

elif [ "$NETWORK" = "mainnet" ]; then
    echo "🌐 Configurando red mainnet..."
    soroban network add \
        --global mainnet \
        --rpc-url https://horizon.stellar.org \
        --network-passphrase "Public Global Stellar Network ; September 2015" 2>/dev/null || true

    if ! soroban keys show deployer --network mainnet &> /dev/null; then
        echo "❌ Necesitas configurar una identidad para mainnet manualmente"
        echo "Usa: soroban keys generate --global deployer --network mainnet"
        exit 1
    fi
fi

# Desplegar contrato
echo "🚀 Desplegando contrato en $NETWORK..."
CONTRACT_ID=$(soroban contract deploy \
    --wasm target/wasm32-unknown-unknown/release/agrocoin_contract.wasm \
    --source deployer \
    --network $NETWORK)

echo "✅ Contrato desplegado exitosamente!"
echo "📋 Contract ID: $CONTRACT_ID"

# Guardar el ID del contrato
echo $CONTRACT_ID > contract_id_$NETWORK.txt
echo "💾 Contract ID guardado en: contract_id_$NETWORK.txt"

# Inicializar contrato
echo "🔧 Inicializando contrato..."
ADMIN_ADDRESS=$(soroban keys address deployer)

soroban contract invoke \
    --id $CONTRACT_ID \
    --source deployer \
    --network $NETWORK \
    -- initialize \
    --admin $ADMIN_ADDRESS

echo "✅ Contrato inicializado con admin: $ADMIN_ADDRESS"

# Mostrar información útil
echo ""
echo "🎉 ¡Despliegue completado!"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "Contract ID: $CONTRACT_ID"
echo "Network: $NETWORK"
echo "Admin: $ADMIN_ADDRESS"
echo ""
echo "📖 Ejemplos de uso:"
echo ""
echo "# Crear proyecto:"
echo "soroban contract invoke --id $CONTRACT_ID --source deployer --network $NETWORK -- create_project --owner $ADMIN_ADDRESS --name \"Mi Proyecto\" --description \"Descripción\" --funding_goal 1000000000 --min_investment 10000000 --expected_roi 1800 --duration_months 12"
echo ""
echo "# Ver proyecto:"
echo "soroban contract invoke --id $CONTRACT_ID --source deployer --network $NETWORK -- get_project --project_id 1"
echo ""
echo "# Invertir:"
echo "soroban contract invoke --id $CONTRACT_ID --source deployer --network $NETWORK -- invest --investor $ADMIN_ADDRESS --project_id 1 --amount 500000000"
