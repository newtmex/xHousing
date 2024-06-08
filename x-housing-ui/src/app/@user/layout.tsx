'use client';

import React, { useCallback } from 'react';
import MobileMenu from './components/MobileMenu';
import MainMenu from './components/MainMenu';
import TopBar from './components/TopBar';
import Sidebar from './components/Sidebar';
import { useContentPanel, useWindowWidthChange } from '@/hooks';

export default function UserLayout({
  children
}: {
  children: React.ReactNode;
}) {
  const {
    contentPanelActive,
    showContentPanel,
    hideContentPanel,
    toggleContentPanel
  } = useContentPanel();

  useWindowWidthChange(
    useCallback(() => {
      !contentPanelActive && window.innerWidth >= 1150 && showContentPanel();
      contentPanelActive && window.innerWidth < 1150 && hideContentPanel();
    }, [contentPanelActive])
  );

  return (
    <div
      className={`all-wrapper with-side-panel solid-bg-all${
        contentPanelActive ? ' content-panel-active' : ''
      }`}
    >
      <div className='layout-w'>
        <MobileMenu />
        <MainMenu />

        <div className='content-w'>
          <TopBar />
          <div onClick={toggleContentPanel} className='content-panel-toggler'>
            <i className='os-icon os-icon-grid-squares-22'></i>
            <span>Sidebar</span>
          </div>
          <div className='content-i'>
            <div className='content-box' style={{ minHeight: '95vh' }}>
              {children}
            </div>

            <Sidebar />
          </div>
        </div>
      </div>
      <div className='display-type'></div>
    </div>
  );
}
