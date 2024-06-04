'use client';

import { DappProvider } from '@/components';
import { apiTimeout, environment, walletConnectV2ProjectId } from '@/config';
import type { PropsWithChildren, ReactNode } from 'react';

const AppContent = ({ children }: PropsWithChildren) => {
  return (
    <DappProvider
      environment={environment}
      customNetworkConfig={{
        name: 'customConfig',
        apiTimeout,
        walletConnectV2ProjectId
      }}
      dappConfig={{
        isSSR: true,
        shouldUseWebViewProvider: true,
        logoutRoute: 'unlock'
      }}
      customComponents={{
        transactionTracker: {
          props: {
            onSuccess: (sessionId: string) => {
              console.log(`Session ${sessionId} successfully completed`);
            },
            onFail: (sessionId: string, errorMessage: string) => {
              console.log(`Session ${sessionId} failed. ${errorMessage ?? ''}`);
            }
          }
        }
      }}
    >
      {children}
    </DappProvider>
  );
};

export default function App({ children }: { children: ReactNode }) {
  return <AppContent>{children}</AppContent>;
}
