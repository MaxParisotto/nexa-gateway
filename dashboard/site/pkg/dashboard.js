// Dashboard JavaScript

document.addEventListener('DOMContentLoaded', function() {
  // Create the main dashboard container
  const mainElement = document.getElementById('main');
  const dashboardContainer = document.createElement('div');
  dashboardContainer.className = 'dashboard-container';
  
  // Create header
  const header = document.createElement('header');
  header.className = 'dashboard-header';
  
  const logo = document.createElement('div');
  logo.className = 'logo';
  logo.textContent = 'Nexa Gateway';
  
  header.appendChild(logo);
  dashboardContainer.appendChild(header);
  
  // Create content area
  const content = document.createElement('div');
  content.className = 'dashboard-content';
  
  // Create sidebar
  const sidebar = document.createElement('aside');
  sidebar.className = 'dashboard-sidebar';
  
  const navList = document.createElement('ul');
  navList.className = 'sidebar-nav-list';
  
  // Add navigation items
  const navItems = [
    { name: 'Dashboard', path: '/' },
    { name: 'Metrics', path: '/metrics' },
    { name: 'Settings', path: '/settings' },
    { name: 'Logs', path: '/logs' }
  ];
  
  navItems.forEach(item => {
    const li = document.createElement('li');
    li.className = 'sidebar-nav-item';
    if (window.location.pathname === item.path) {
      li.classList.add('active');
    }
    
    const a = document.createElement('a');
    a.href = item.path;
    a.textContent = item.name;
    
    li.appendChild(a);
    navList.appendChild(li);
  });
  
  sidebar.appendChild(navList);
  content.appendChild(sidebar);
  
  // Create main content area
  const mainContent = document.createElement('main');
  mainContent.className = 'main-content';
  
  const heading = document.createElement('h1');
  heading.textContent = 'Nexa Gateway Dashboard';
  
  const welcomeMessage = document.createElement('p');
  welcomeMessage.textContent = 'Welcome to the Nexa Gateway control dashboard. This interface allows you to monitor and manage your gateway services.';
  
  // Create status overview section
  const statusSection = document.createElement('section');
  const statusHeading = document.createElement('h2');
  statusHeading.textContent = 'System Status';
  
  const statusOverview = document.createElement('div');
  statusOverview.className = 'status-overview';
  
  // Add status cards
  const statusCards = [
    { title: 'API Server', status: 'Online', value: '100%' },
    { title: 'Database', status: 'Online', value: '99.8%' },
    { title: 'Vector DB', status: 'Online', value: '99.5%' },
    { title: 'Auth Service', status: 'Online', value: '100%' }
  ];
  
  statusCards.forEach(card => {
    const cardElement = document.createElement('div');
    cardElement.className = 'dashboard-card';
    
    const cardHeader = document.createElement('div');
    cardHeader.className = 'card-header';
    
    const cardTitle = document.createElement('h3');
    cardTitle.className = 'card-title';
    cardTitle.textContent = card.title;
    
    cardHeader.appendChild(cardTitle);
    
    const cardContent = document.createElement('div');
    cardContent.className = 'card-content';
    
    const statusIndicator = document.createElement('div');
    statusIndicator.className = 'status-indicator';
    
    const statusDot = document.createElement('span');
    statusDot.className = 'status-dot';
    statusDot.style.backgroundColor = '#2ecc71'; // Green for online
    
    const statusLabel = document.createElement('span');
    statusLabel.className = 'status-label';
    statusLabel.textContent = card.status;
    
    statusIndicator.appendChild(statusDot);
    statusIndicator.appendChild(statusLabel);
    
    const metricValue = document.createElement('div');
    metricValue.className = 'metric-value';
    metricValue.textContent = card.value;
    
    cardContent.appendChild(statusIndicator);
    cardContent.appendChild(metricValue);
    
    cardElement.appendChild(cardHeader);
    cardElement.appendChild(cardContent);
    
    statusOverview.appendChild(cardElement);
  });
  
  statusSection.appendChild(statusHeading);
  statusSection.appendChild(statusOverview);
  
  mainContent.appendChild(heading);
  mainContent.appendChild(welcomeMessage);
  mainContent.appendChild(statusSection);
  
  content.appendChild(mainContent);
  dashboardContainer.appendChild(content);
  
  // Add the dashboard to the page
  mainElement.appendChild(dashboardContainer);
});

// This file is used by the Leptos application when it's compiled to WASM
// For now we're just using the static HTML version
console.log("Dashboard JS loaded");
