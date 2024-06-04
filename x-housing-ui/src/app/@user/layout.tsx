import React from 'react';
import MobileMenu from './components/MobileMenu';
import MainMenu from './components/MainMenu';
import TopBar from './components/TopBar';
import Sidebar from './components/Sidebar';

export default function UserLayout({
  children
}: {
  children: React.ReactNode;
}) {
  return (
    <div className='all-wrapper with-side-panel solid-bg-all'>
      <div className='layout-w'>
        <MobileMenu />
        <MainMenu />

        <div className='content-w'>
          <TopBar />
          <div className='content-panel-toggler'>
            <i className='os-icon os-icon-grid-squares-22'></i>
            <span>Sidebar</span>
          </div>
          <div className='content-i'>
            <div className='content-box'>{children}</div>

            <Sidebar />
          </div>
        </div>
      </div>
      <div className='display-type'></div>
    </div>
  );
}
