

let isAuthenticated = false;
let currentUser = null;

document.addEventListener('DOMContentLoaded', function() {
    initializeApp();
    setupMobileMenu();
    checkAuthStatus();
});

function initializeApp() {
    console.log('AgroCoin initialized');
}

function setupMobileMenu() {
    const mobileMenuBtn = document.getElementById('mobileMenuBtn');
    const mobileMenu = document.getElementById('mobileMenu');

    if (mobileMenuBtn && mobileMenu) {
        mobileMenuBtn.addEventListener('click', function() {
            mobileMenu.classList.toggle('hidden');
        });
    }
}

function checkAuthStatus() {
    const token = localStorage.getItem('authToken');
    const userData = localStorage.getItem('userData');

    if (token && userData) {
        try {
            currentUser = JSON.parse(userData);
            isAuthenticated = true;
            updateNavigation();
        } catch (error) {
            console.error('Error parsing user data:', error);
            logout();
        }
    } else {
        isAuthenticated = false;
        updateNavigation();
    }
}

function updateNavigation() {
    const guestNav = document.querySelector('nav:not(#authNav)');
    const authNav = document.getElementById('authNav');
    const userWelcome = document.getElementById('userWelcome');

    if (isAuthenticated && currentUser) {
        
        if (guestNav) {
            const loginBtn = guestNav.querySelector('a[href*="auth"]');
            if (loginBtn) loginBtn.style.display = 'none';
        }

        
        if (authNav) {
            authNav.classList.remove('hidden');
            authNav.classList.add('flex');
        }

        
        if (userWelcome) {
            userWelcome.textContent = `Hola, ${currentUser.name || currentUser.email}`;
        }

        
        updateDashboardLinks();
    } else {
        
        if (guestNav) {
            const loginBtn = guestNav.querySelector('a[href*="auth"]');
            if (loginBtn) loginBtn.style.display = 'inline-block';
        }

        
        if (authNav) {
            authNav.classList.add('hidden');
            authNav.classList.remove('flex');
            authNav.style.display = 'none'; 
        }

        
        if (userWelcome) {
            userWelcome.textContent = '';
        }
    }
}

function updateDashboardLinks() {
    if (!currentUser) return;

    const dashboardLink = document.querySelector('a[href*="dashboard"]');
    if (dashboardLink) {
        if (currentUser.type === 'productor') {
            dashboardLink.href = getDashboardPath('dashboard-productor.html');
        } else {
            dashboardLink.href = getDashboardPath('dashboard-inversor.html');
        }
    }
}

function getDashboardPath(dashboardFile) {
    const currentPath = window.location.pathname;

    if (currentPath.includes('/pages/')) {
        return `../dashboards/${dashboardFile}`;
    } else if (currentPath.includes('/dashboards/')) {
        return dashboardFile;
    } else if (currentPath.includes('/auth/')) {
        return `../dashboards/${dashboardFile}`;
    } else {
        return `dashboards/${dashboardFile}`;
    }
}
function login(email, password, userType) {
    return new Promise((resolve, reject) => {
        setTimeout(() => {
            if (email && password) {
                const userData = {
                    id: Date.now(),
                    email: email,
                    name: email.split('@')[0],
                    type: userType || 'inversor'
                };

                const token = 'fake-jwt-token-' + Date.now();

                localStorage.setItem('authToken', token);
                localStorage.setItem('userData', JSON.stringify(userData));

                currentUser = userData;
                isAuthenticated = true;

                resolve(userData);
            } else {
                reject(new Error('Credenciales inválidas'));
            }
        }, 1000);
    });
}

function logout() {
    localStorage.removeItem('authToken');
    localStorage.removeItem('userData');

    currentUser = null;
    isAuthenticated = false;

    updateNavigation();

    showAlert('Sesión cerrada exitosamente', 'success');

    setTimeout(() => {
        window.location.href = getHomePath();
    }, 1500);
}

function getHomePath() {
    const currentPath = window.location.pathname;

    if (currentPath.includes('/pages/')) {
        return '../index.html';
    } else if (currentPath.includes('/dashboards/')) {
        return '../index.html';
    } else if (currentPath.includes('/auth/')) {
        return '../index.html';
    } else {
        return 'index.html';
    }
}

function showAlert(message, type = 'info') {
    const existingAlert = document.querySelector('.alert-notification');
    if (existingAlert) {
        existingAlert.remove();
    }

    const alert = document.createElement('div');
    alert.className = `alert-notification fixed top-4 right-4 z-50 p-4 rounded-lg shadow-lg max-w-sm ${getAlertClasses(type)}`;
    alert.innerHTML = `
        <div class="flex items-center">
            <span class="flex-1">${message}</span>
            <button onclick="this.parentElement.parentElement.remove()" class="ml-2 text-current opacity-70 hover:opacity-100">
                <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd"></path>
                </svg>
            </button>
        </div>
    `;

    document.body.appendChild(alert);

    setTimeout(() => {
        if (alert.parentElement) {
            alert.remove();
        }
    }, 5000);
}


function getAlertClasses(type) {
    const classes = {
        success: 'bg-green-100 text-green-800 border border-green-300',
        error: 'bg-red-100 text-red-800 border border-red-300',
        warning: 'bg-yellow-100 text-yellow-800 border border-yellow-300',
        info: 'bg-blue-100 text-blue-800 border border-blue-300'
    };

    return classes[type] || classes.info;
}

function requireAuth() {
    if (!isAuthenticated) {
        showAlert('Debes iniciar sesión para acceder a esta página', 'warning');
        setTimeout(() => {
            window.location.href = getAuthPath();
        }, 2000);
        return false;
    }
    return true;
}

function getAuthPath() {
    const currentPath = window.location.pathname;

    if (currentPath.includes('/pages/')) {
        return '../auth/auth.html';
    } else if (currentPath.includes('/dashboards/')) {
        return '../auth/auth.html';
    } else {
        return 'auth/auth.html';
    }
}

function formatCurrency(amount, currency = 'USD') {
    return new Intl.NumberFormat('es-ES', {
        style: 'currency',
        currency: currency
    }).format(amount);
}


function formatDate(date, locale = 'es-ES') {
    return new Date(date).toLocaleDateString(locale, {
        year: 'numeric',
        month: 'long',
        day: 'numeric'
    });
}

function isValidEmail(email) {
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    return emailRegex.test(email);
}

function generateRandomColor() {
    const colors = [
        '#16a34a', '#3b82f6', '#f59e0b', '#ef4444', '#8b5cf6',
        '#06b6d4', '#f97316', '#84cc16', '#ec4899', '#6366f1'
    ];
    return colors[Math.floor(Math.random() * colors.length)];
}

window.AgroCoin = {
    login,
    logout,
    checkAuthStatus,
    requireAuth,
    showAlert,
    formatCurrency,
    formatDate,
    isValidEmail,
    generateRandomColor,
    isAuthenticated: () => isAuthenticated,
    getCurrentUser: () => currentUser
};
