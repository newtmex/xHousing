'use client';

import {
  DappProvider,
  NotificationModal,
  SignTransactionsModals,
  TransactionsToastList
} from '@/components';
import BootstrapClient from '@/components/BootstrapClient';
import {
  apiTimeout,
  environment,
  walletConnectV2ProjectId,
} from '@/config';
import type { PropsWithChildren, ReactNode } from 'react';
import { SWRConfig } from 'swr';

const AppContent = ({ children }: PropsWithChildren) => {
  return (
    <DappProvider
      environment={environment}
      customNetworkConfig={{
        name: 'customConfig',
        apiTimeout,
        walletConnectV2ProjectId
        // TODO uncomment when working on localnet
        // skipFetchFromServer: true,
        // walletConnectDeepLink:
        //   'https://maiar.page.link/?apn=com.elrond.maiar.wallet&isi=1519405832&ibi=com.elrond.maiar.wallet&link=https://xportal.com/',
        // walletConnectBridgeAddresses: ['https://bridge.walletconnect.org'],
        // walletConnectV2RelayAddresses: ['wss://relay.walletconnect.com'],
        // walletAddress,
        // apiAddress
      }}
      dappConfig={{
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
      <TransactionsToastList />
      <NotificationModal />
      <SignTransactionsModals />
      <SWRConfig> {children}</SWRConfig>
      <BootstrapClient />
    </DappProvider>
  );
};

export default function App({ children }: { children: ReactNode }) {
  return <AppContent> {children}</AppContent>;
}
