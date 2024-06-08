import { useContentPanel } from '@/hooks';
import EcosystemTransactions from './EcosystemTransactions';
import QuickSwap from './QuickSwap';
import ReferralCard from './ReferralCard';

export default function Sidebar() {
  const { hideContentPanel } = useContentPanel();

  return (
    <div className='content-panel compact color-scheme-dark'>
      <div onClick={hideContentPanel} className='content-panel-close'>
        <i className='os-icon os-icon-close'></i>
      </div>
      <div className='d-xxl-none'>
        <ReferralCard />
      </div>
      <QuickSwap />
      <EcosystemTransactions />
    </div>
  );
}
