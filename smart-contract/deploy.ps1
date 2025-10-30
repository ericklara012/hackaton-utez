# Script de despliegue automatizado para AgroCoin Smart Contract en PowerShell
# Uso: .\deploy.ps1 [testnet|mainnet]

param(
    [string]$Network = "testnet"
)

$ContractName = "agrocoin-contract"

Write-Host "🚀 Desplegando AgroCoin Smart Contract en $Network..." -ForegroundColor Green

# Verificar que Soroban CLI esté instalado
try {
    $sorobanVersion = & soroban --version 2>$null
    Write-Host "✅ Soroban CLI encontrado: $sorobanVersion" -ForegroundColor Green
}
catch {
    Write-Host "❌ Soroban CLI no está instalado. Instálalo con:" -ForegroundColor Red
    Write-Host "cargo install --locked soroban-cli" -ForegroundColor Yellow
    exit 1
}

# Verificar que el target wasm32 esté instalado
$targets = & rustup target list --installed
if ($targets -notcontains "wasm32-unknown-unknown") {
    Write-Host "📦 Instalando target wasm32-unknown-unknown..." -ForegroundColor Yellow
    & rustup target add wasm32-unknown-unknown
}

# Compilar el contrato
Write-Host "🔨 Compilando contrato..." -ForegroundColor Cyan
& cargo build --target wasm32-unknown-unknown --release

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Error en la compilación" -ForegroundColor Red
    exit 1
}

# Optimizar WASM
Write-Host "⚡ Optimizando WASM..." -ForegroundColor Cyan
& soroban contract optimize --wasm target/wasm32-unknown-unknown/release/agrocoin_contract.wasm

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Error en la optimización" -ForegroundColor Red
    exit 1
}

# Configurar red
if ($Network -eq "testnet") {
    Write-Host "🌐 Configurando red testnet..." -ForegroundColor Cyan
    
    try {
        & soroban network add --global testnet --rpc-url https://soroban-testnet.stellar.org:443 --network-passphrase "Test SDF Network ; September 2015" 2>$null
    }
    catch {
        # Red ya existe, continuar
    }

    # Verificar si la identidad existe
    try {
        $deployerAddress = & soroban keys show deployer --network testnet 2>$null
        Write-Host "✅ Identidad 'deployer' ya existe: $deployerAddress" -ForegroundColor Green
    }
    catch {
        Write-Host "🔑 Generando nueva identidad para deployment..." -ForegroundColor Yellow
        & soroban keys generate --global deployer --network testnet

        Write-Host "💰 Obteniendo fondos de testnet..." -ForegroundColor Yellow
        & soroban keys fund deployer --network testnet
        
        if ($LASTEXITCODE -ne 0) {
            Write-Host "❌ Error obteniendo fondos de testnet" -ForegroundColor Red
            exit 1
        }
    }
}
elseif ($Network -eq "mainnet") {
    Write-Host "🌐 Configurando red mainnet..." -ForegroundColor Cyan
    
    try {
        & soroban network add --global mainnet --rpc-url https://horizon.stellar.org --network-passphrase "Public Global Stellar Network ; September 2015" 2>$null
    }
    catch {
        # Red ya existe, continuar
    }

    try {
        $deployerAddress = & soroban keys show deployer --network mainnet 2>$null
        Write-Host "✅ Identidad 'deployer' existe: $deployerAddress" -ForegroundColor Green
    }
    catch {
        Write-Host "❌ Necesitas configurar una identidad para mainnet manualmente" -ForegroundColor Red
        Write-Host "Usa: soroban keys generate --global deployer --network mainnet" -ForegroundColor Yellow
        exit 1
    }
}

# Desplegar contrato
Write-Host "🚀 Desplegando contrato en $Network..." -ForegroundColor Cyan
$ContractId = & soroban contract deploy --wasm target/wasm32-unknown-unknown/release/agrocoin_contract.wasm --source deployer --network $Network

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Error en el deployment" -ForegroundColor Red
    exit 1
}

Write-Host "✅ Contrato desplegado exitosamente!" -ForegroundColor Green
Write-Host "📋 Contract ID: $ContractId" -ForegroundColor Yellow

# Guardar el ID del contrato
$ContractId | Out-File -FilePath "contract_id_$Network.txt" -Encoding UTF8
Write-Host "💾 Contract ID guardado en: contract_id_$Network.txt" -ForegroundColor Green

# Inicializar contrato
Write-Host "🔧 Inicializando contrato..." -ForegroundColor Cyan
$AdminAddress = & soroban keys address deployer

& soroban contract invoke --id $ContractId --source deployer --network $Network -- initialize --admin $AdminAddress

if ($LASTEXITCODE -ne 0) {
    Write-Host "❌ Error en la inicialización" -ForegroundColor Red
    exit 1
}

Write-Host "✅ Contrato inicializado con admin: $AdminAddress" -ForegroundColor Green

# Mostrar información útil
Write-Host ""
Write-Host "🎉 ¡Despliegue completado!" -ForegroundColor Green
Write-Host "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━" -ForegroundColor Gray
Write-Host "Contract ID: $ContractId" -ForegroundColor White
Write-Host "Network: $Network" -ForegroundColor White
Write-Host "Admin: $AdminAddress" -ForegroundColor White
Write-Host ""
Write-Host "📖 Ejemplos de uso:" -ForegroundColor Cyan
Write-Host ""
Write-Host "# Crear proyecto:" -ForegroundColor Gray
Write-Host "soroban contract invoke --id $ContractId --source deployer --network $Network -- create_project --owner $AdminAddress --name `"Mi Proyecto`" --description `"Descripción`" --funding_goal 1000000000 --min_investment 10000000 --expected_roi 1800 --duration_months 12" -ForegroundColor White
Write-Host ""
Write-Host "# Ver proyecto:" -ForegroundColor Gray
Write-Host "soroban contract invoke --id $ContractId --source deployer --network $Network -- get_project --project_id 1" -ForegroundColor White
Write-Host ""
Write-Host "# Invertir:" -ForegroundColor Gray
Write-Host "soroban contract invoke --id $ContractId --source deployer --network $Network -- invest --investor $AdminAddress --project_id 1 --amount 500000000" -ForegroundColor White