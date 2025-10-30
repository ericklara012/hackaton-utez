# AgroCoin Smart Contract - Soroban/Stellar

Smart contract en Rust para la plataforma de inversión agrícola AgroCoin en la red Stellar.

## 🚀 Características

- **Creación de Proyectos**: Los productores pueden crear proyectos agrícolas con metas de financiamiento
- **Inversiones Descentralizadas**: Los inversores pueden financiar proyectos con montos mínimos
- **ROI Transparente**: Seguimiento automático de retornos de inversión
- **Gestión de Fondos**: Distribución automática de ganancias
- **Controles de Admin**: Funciones de emergencia y gestión

## 📦 Estructura del Contrato

### Estructuras Principales

```rust
pub struct Project {
    pub id: u64,
    pub owner: Address,           // Productor
    pub name: String,             // Nombre del proyecto
    pub funding_goal: i128,       // Meta de financiamiento
    pub current_funding: i128,    // Fondos actuales
    pub min_investment: i128,     // Inversión mínima
    pub expected_roi: u32,        // ROI esperado en %
    pub duration_months: u32,     // Duración en meses
    pub is_active: bool,          // Estado activo
    pub is_funded: bool,          // Financiamiento completo
}

pub struct Investment {
    pub investor: Address,
    pub project_id: u64,
    pub amount: i128,             // Cantidad invertida
    pub timestamp: u64,           // Timestamp
    pub claimed_returns: i128,    // Retornos reclamados
}
```

## 🛠️ Funciones Principales

### Para Administradores
- `initialize(admin)` - Inicializar contrato
- `pause_project(project_id)` - Pausar proyecto

### Para Productores
- `create_project(...)` - Crear nuevo proyecto
- `withdraw_funds(project_id)` - Retirar fondos
- `get_user_projects(user)` - Ver proyectos propios

### Para Inversores
- `invest(project_id, amount)` - Invertir en proyecto
- `claim_returns(project_id)` - Reclamar ganancias
- `get_user_investments(user)` - Ver inversiones

### Consultas Públicas
- `get_project(project_id)` - Información del proyecto
- `get_project_stats(project_id)` - Estadísticas
- `get_investment(investor, project_id)` - Ver inversión específica

## 🔧 Compilación y Despliegue

### Requisitos
```bash
# Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Instalar Soroban CLI
cargo install --locked soroban-cli

# Instalar target wasm32
rustup target add wasm32-unknown-unknown
```

### Compilar
```bash
cd smart-contract
cargo build --target wasm32-unknown-unknown --release
```

### Optimizar WASM
```bash
soroban contract optimize --wasm target/wasm32-unknown-unknown/release/agrocoin_contract.wasm
```

### Desplegar en Testnet
```bash
# Configurar red de testnet
soroban network add \
  --global testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015"

# Configurar identidad
soroban keys generate --global alice --network testnet

# Obtener fondos de testnet
soroban keys fund alice --network testnet

# Desplegar contrato
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/agrocoin_contract.wasm \
  --source alice \
  --network testnet
```

## 📋 Ejemplo de Uso

### 1. Inicializar Contrato
```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source alice \
  --network testnet \
  -- initialize \
  --admin ADMIN_ADDRESS
```

### 2. Crear Proyecto
```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source producer \
  --network testnet \
  -- create_project \
  --owner PRODUCER_ADDRESS \
  --name "Cultivos Orgánicos Valle Verde" \
  --description "Proyecto de agricultura orgánica sostenible" \
  --funding_goal 1000000000 \
  --min_investment 10000000 \
  --expected_roi 1800 \
  --duration_months 12
```

### 3. Invertir en Proyecto
```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source investor \
  --network testnet \
  -- invest \
  --investor INVESTOR_ADDRESS \
  --project_id 1 \
  --amount 500000000
```

### 4. Consultar Proyecto
```bash
soroban contract invoke \
  --id CONTRACT_ID \
  --source alice \
  --network testnet \
  -- get_project \
  --project_id 1
```

## 🧪 Tests

Ejecutar tests unitarios:
```bash
cargo test
```

## 🔐 Seguridad

- **Autenticación**: Todas las funciones críticas requieren autenticación
- **Validaciones**: Montos, ROI y duraciones son validados
- **Pausas de Emergencia**: Admin puede pausar proyectos
- **Límites**: ROI máximo del 100%, duración máxima de 60 meses

## 💰 Consideraciones Económicas

- Todas las cantidades están en **stroops** (1 XLM = 10^7 stroops)
- Inversión mínima configurable por proyecto
- ROI expresado en puntos básicos (1800 = 18%)
- Fondos bloqueados hasta completar duración del proyecto

## 🌍 Integración Frontend

Para conectar con tu frontend HTML/JS, usa el SDK de Soroban:

```javascript
import { SorobanRpc, Contract, TransactionBuilder } from '@stellar/stellar-sdk';

const server = new SorobanRpc.Server('https://soroban-testnet.stellar.org:443');
const contract = new Contract(CONTRACT_ADDRESS);

// Llamar función del contrato
const operation = contract.call('get_project', project_id);
```

## 📄 Licencia

MIT License - Proyecto de código abierto para inversiones agrícolas descentralizadas.
