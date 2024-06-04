import React from 'react';
import { AuthRedirectWrapper } from './components/AuthRedirectWrapper';

export default function AuthLayout({
  children
}: {
  children: React.ReactNode;
}) {
  return (
    <AuthRedirectWrapper requireAuth={false}>
      <div
        style={{ overflow: 'scroll' }}
        className='all-wrapper menu-side with-pattern'
      >
        <div className='layout-w'>
          <div className='auth-box-w wider centered'>{children}</div>
        </div>
      </div>
    </AuthRedirectWrapper>
  );
}
