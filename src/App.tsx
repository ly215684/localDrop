import { useState, useEffect } from 'react';
import { Home, Send, Settings } from 'lucide-react';
import { HomePage } from '@/pages/HomePage';
import { TransferPage } from '@/pages/TransferPage';
import { SettingsPage } from '@/pages/SettingsPage';
import { useTransferEvents } from '@/hooks/useTransferEvents';
import './index.css';

type Page = 'home' | 'transfers' | 'settings';

function App() {
  const [currentPage, setCurrentPage] = useState<Page>('home');
  
  useTransferEvents();

  useEffect(() => {
    document.title = 'LocalDrop - 局域网文件传输';
  }, []);

  const handleNavigate = (page: string) => {
    if (page === '/') {
      setCurrentPage('home');
    } else if (page === '/transfers') {
      setCurrentPage('transfers');
    } else if (page === '/settings') {
      setCurrentPage('settings');
    }
  };

  const tabs = [
    { id: 'home' as Page, label: '设备', icon: Home },
    { id: 'transfers' as Page, label: '传输', icon: Send },
    { id: 'settings' as Page, label: '设置', icon: Settings },
  ];

  const renderPage = () => {
    switch (currentPage) {
      case 'home':
        return <HomePage onNavigate={handleNavigate} />;
      case 'transfers':
        return <TransferPage onNavigate={handleNavigate} />;
      case 'settings':
        return <SettingsPage onNavigate={handleNavigate} />;
      default:
        return <HomePage onNavigate={handleNavigate} />;
    }
  };

  return (
    <div className="h-screen flex flex-col bg-gray-100">
      <div className="flex-1 overflow-hidden">
        {renderPage()}
      </div>
      
      <div className="flex items-center justify-around bg-white border-t border-gray-200 px-2 py-2">
        {tabs.map((tab) => {
          const Icon = tab.icon;
          const isActive = currentPage === tab.id;
          return (
            <button
              key={tab.id}
              onClick={() => handleNavigate(`/${tab.id === 'home' ? '' : tab.id}`)}
              className={`flex flex-col items-center gap-1 px-6 py-2 rounded-lg transition-colors ${
                isActive 
                  ? 'text-blue-500 bg-blue-50' 
                  : 'text-gray-500 hover:text-gray-700 hover:bg-gray-100'
              }`}
            >
              <Icon className="w-5 h-5" />
              <span className="text-xs font-medium">{tab.label}</span>
            </button>
          );
        })}
      </div>
    </div>
  );
}

export default App;
