'use client';

import { useGetAccountInfo } from '@/hooks';
// @ts-ignore
import blockies from 'blockies-identicon';
import { useEffect, useState } from 'react';

export default function UserAvatar() {
  const { publicKey } = useGetAccountInfo();
  const [dataUrl, setDataUrl] = useState();

  useEffect(() => {
    setDataUrl(
      blockies
        .create({ seed: publicKey || '----', size: 8, scale: 16 })
        .toDataURL()
    );
  }, [publicKey]);

  if (!dataUrl) {
    return null;
  }

  return (
    <div className='avatar-w'>
      <img alt='' src={dataUrl} />
    </div>
  );
}
