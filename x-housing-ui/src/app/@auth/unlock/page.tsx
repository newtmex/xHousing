import { Metadata } from 'next';
import Auth from '.';

export const metadata: Metadata = {
  title: 'xHousing | Connect Wallet'
};

export default function UnlockPage() {
  return <Auth />;
}
