'use client';

import AppDownload from '../components/AppDownload';
import BlogSummary from '../components/BlogSummary';
import PortfolioBalance from '../components/PortfolioBalance';
import PortfolioDistribution from '../components/PortfolioDistribution';
import Referrals from '../components/Referrals';
import RentDividends from '../components/RentDividends';
import UserEcosystemTransactions from '../components/UserEcosystemTransactions';

export default function Dashboard() {
  return (
    <>
      <div className='row'>
        <PortfolioBalance />
        <PortfolioDistribution />
        <AppDownload />
      </div>
      <RentDividends />
      <div className='row'>
        <div className='col-sm-8'>
          <Referrals />
          <BlogSummary />
        </div>
        <div className='col-sm-4'>
          <UserEcosystemTransactions />
        </div>
      </div>
    </>
  );
}
