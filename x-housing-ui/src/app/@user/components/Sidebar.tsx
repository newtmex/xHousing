import EcosystemTransactions from './EcosystemTransactions';
import QuickSwap from './QuickSwap';
import ReferralCard from './ReferralCard';

export default function Sidebar() {
  return (
    <div className='content-panel compact color-scheme-dark'>
      <div className='content-panel-close'>
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
