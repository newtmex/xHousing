'use client';
import type {
  ExtensionLoginButtonPropsType,
  WebWalletLoginButtonPropsType,
  WalletConnectLoginButtonPropsType
} from '@multiversx/sdk-dapp/UI';
import { nativeAuth } from '@/config';

import {
  ExtensionLoginButton,
  WalletConnectLoginButton,
  WebWalletLoginButton as WebWalletUrlLoginButton,
  CrossWindowLoginButton
} from '@/components';
import { useRouter } from 'next/navigation';

type CommonPropsType =
  | ExtensionLoginButtonPropsType
  | WebWalletLoginButtonPropsType
  | WalletConnectLoginButtonPropsType;

// choose how you want to configure connecting to the web wallet
const USE_WEB_WALLET_CROSS_WINDOW = false;

const WebWalletLoginButton = USE_WEB_WALLET_CROSS_WINDOW
  ? CrossWindowLoginButton
  : WebWalletUrlLoginButton;

export default function Auth() {
  const router = useRouter();
  const commonProps: CommonPropsType = {
    callbackRoute: '/',
    nativeAuth,
    onLoginRedirect: () => {
      console.log('loging');
      router.push('/');
    }
  };

  return (
    <div className='p-15'>
      <h5 className='auth-header'>xHousing Gateway</h5>
      <div className='logged-user-w'>
        <div className='logged-user-name'>{/* TODO Short description */}</div>
      </div>
      <div className='mb-3 form-group'>
        <div className='pre-icon os-icon os-icon-fingerprint'></div>
        <label htmlFor={''}>Please select your login method:</label>
      </div>
      <div className='buttons-w'>
        <WalletConnectLoginButton
          loginButtonText='xPortal App'
          {...commonProps}
        />

        <ExtensionLoginButton
          loginButtonText='DeFi Wallet'
          buttonClassName='btn btn-secondary'
          {...commonProps}
        />
        <WebWalletLoginButton
          loginButtonText='Web Wallet'
          className='btn btn-primary'
          {...commonProps}
        />
      </div>
    </div>
  );
}
