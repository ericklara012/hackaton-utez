# 🌱 AgroCoin - Plataforma de Inversiones Agrícolas Descentralizadas

[![Stellar](https://img.shields.io/badge/Blockchain-Stellar-blue)](https://stellar.org/)
[![Soroban](https://img.shields.io/badge/Smart%20Contract-Soroban-green)](https://soroban.stellar.org/)
[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

Una plataforma descentralizada que conecta inversionistas con productores agrícolas, permitiendo el financiamiento transparente y eficiente de proyectos agrícolas sostenibles.

## 🚀 Características Principales

- **Smart Contract en Soroban**: Contrato inteligente desplegado en la red Stellar
- **Inversiones Transparentes**: Seguimiento en tiempo real de proyectos e inversiones
- **ROI Predefinido**: Retornos de inversión claramente establecidos
- **Seguridad**: Validaciones y controles de acceso implementados
- **Interfaz Web Intuitiva**: Dashboard para productores e inversionistas

## 📊 Información del Smart Contract

- **Contract ID**: `CCPATMQZHDUEK25TY5LQWWNN3PJQGABHNYCMUWW6K7ZUDO4NMEZE6R2Z`
- **Red**: Stellar Testnet
- **Explorador**: [Ver en Stellar Expert](https://stellar.expert/explorer/testnet/contract/CCPATMQZHDUEK25TY5LQWWNN3PJQGABHNYCMUWW6K7ZUDO4NMEZE6R2Z)

## 🏗️ Arquitectura del Proyecto

```
hackaton_utez/
├── 📱 Frontend (Web)
│   ├── index.html              # Página principal
│   ├── auth.html              # Autenticación
│   ├── dashboards/            # Dashboards de usuario
│   ├── pages/                 # Páginas específicas
│   ├── assets/                # Recursos (imágenes, JS)
│   └── css/                   # Estilos
│
└── 🔗 Smart Contract (Soroban)
    ├── src/
    │   ├── lib.rs             # Lógica principal del contrato
    │   ├── test.rs            # Tests unitarios
    │   └── bin/main.rs        # Binario principal
    ├── Cargo.toml             # Configuración de Rust
    ├── deploy.ps1             # Script de deployment (PowerShell)
    └── deploy.sh              # Script de deployment (Bash)
```

## 🛠️ Tecnologías Utilizadas

### Frontend
- **HTML5/CSS3/JavaScript**: Interfaz de usuario
- **Responsive Design**: Compatible con dispositivos móviles
- **Local Storage**: Gestión de estado local

### Smart Contract
- **Rust**: Lenguaje de programación
- **Soroban SDK**: Framework para smart contracts en Stellar
- **WASM**: WebAssembly para ejecución eficiente

### Blockchain
- **Stellar Network**: Blockchain de alta velocidad y bajo costo
- **Soroban**: Plataforma de smart contracts de Stellar

## ⚡ Inicio Rápido

### Prerrequisitos

- Rust (1.70+)
- Soroban CLI
- Git
- Navegador web moderno

### Instalación

1. **Clonar el repositorio**
   ```bash
   git clone https://github.com/tu-usuario/agrocoin-hackaton.git
   cd agrocoin-hackaton
   ```

2. **Instalar Soroban CLI**
   ```bash
   cargo install --locked soroban-cli
   ```

3. **Compilar el Smart Contract**
   ```bash
   cd smart-contract
   cargo build --target wasm32-unknown-unknown --release
   ```

4. **Optimizar WASM**
   ```bash
   soroban contract optimize --wasm target/wasm32-unknown-unknown/release/agrocoin_contract.wasm
   ```

### Deployment

#### Usando PowerShell (Windows)
```powershell
cd smart-contract
.\deploy.ps1 testnet
```

#### Usando Bash (Linux/Mac)
```bash
cd smart-contract
./deploy.sh testnet
```

## 📖 Funcionalidades del Smart Contract

### Funciones Principales

| Función | Descripción | Parámetros |
|---------|-------------|------------|
| `initialize` | Inicializa el contrato | `admin: Address` |
| `create_project` | Crea un nuevo proyecto agrícola | `owner, name, description, funding_goal, min_investment, expected_roi, duration_months` |
| `invest` | Realiza una inversión en un proyecto | `investor, project_id, amount` |
| `claim_returns` | Reclama retornos de inversión | `investor, project_id` |
| `get_project` | Obtiene información de un proyecto | `project_id` |
| `get_investment` | Obtiene información de una inversión | `investor, project_id` |

### Funciones de Consulta

- `get_project_count()`: Número total de proyectos
- `get_user_projects(user)`: Proyectos de un usuario
- `get_user_investments(user)`: Inversiones de un usuario
- `get_project_stats(project_id)`: Estadísticas de un proyecto

### Funciones Administrativas

- `pause_project(admin, project_id)`: Pausar un proyecto
- `withdraw_funds(owner, project_id)`: Retirar fondos (solo propietario)

## 🧪 Testing

Ejecutar tests del smart contract:

```bash
cd smart-contract
cargo test
```

### Cobertura de Tests
- ✅ Inicialización del contrato
- ✅ Creación de proyectos
- ✅ Sistema de inversiones
- ✅ Validaciones de seguridad
- ✅ Controles de acceso
- ✅ Funciones administrativas

## 💰 Ejemplos de Uso

### Crear un Proyecto
```bash
soroban contract invoke \
  --id CCPATMQZHDUEK25TY5LQWWNN3PJQGABHNYCMUWW6K7ZUDO4NMEZE6R2Z \
  --source deployer \
  --network testnet \
  -- create_project \
  --owner GBYEI4RWWCKWZQVU6YZZITCDVYZM53UY4YFSXI6LSBS7V6PJJHANVBYJ \
  --name "Cultivos Orgánicos" \
  --description "Proyecto de agricultura sostenible" \
  --funding_goal 1000000000 \
  --min_investment 10000000 \
  --expected_roi 1800 \
  --duration_months 12
```

### Invertir en un Proyecto
```bash
soroban contract invoke \
  --id CCPATMQZHDUEK25TY5LQWWNN3PJQGABHNYCMUWW6K7ZUDO4NMEZE6R2Z \
  --source deployer \
  --network testnet \
  -- invest \
  --investor GBYEI4RWWCKWZQVU6YZZITCDVYZM53UY4YFSXI6LSBS7V6PJJHANVBYJ \
  --project_id 1 \
  --amount 500000000
```

## 🌐 Interfaces Web

### Para Productores
- **Dashboard**: Gestión de proyectos agrícolas
- **Crear Proyecto**: Formulario para nuevos proyectos
- **Seguimiento**: Monitor de inversiones recibidas

### Para Inversionistas
- **Explorar Proyectos**: Catálogo de proyectos disponibles
- **Portfolio**: Seguimiento de inversiones
- **Retornos**: Gestión de ganancias
