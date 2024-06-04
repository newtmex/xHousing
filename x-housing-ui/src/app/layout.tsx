'use client';

import '../styles/global.scss';
import App from '.';
import { usePathname } from 'next/navigation';
import { isAuthRoute } from '@/utils';

export default function RootLayout({
  auth,
  user
}: {
  auth: React.ReactNode;
  user: React.ReactNode;
}) {
  const pathname = usePathname();
  const isAuthPath = isAuthRoute(pathname);

  return (
    <html lang='en'>
      <body
        className={`${
          isAuthPath
            ? 'auth-wrapper'
            : 'menu-position-side menu-side-left full-screen color-scheme-dark with-content-panel'
        }`}
      >
        <App>{isAuthPath ? auth : user}</App>
      </body>
    </html>
  );
}
