import { RoutePath } from '@/utils/routes';
import Link from 'next/link';

export default function PortfolioBalance() {
  return (
    <div className='col-sm-12 col-lg-8 col-xxl-6'>
      <div className='element-balances justify-content-between mobile-full-width'>
        <div className='balance balance-v2'>
          <div className='balance-title'>Your Portfolio Balance</div>
          <div className='balance-value'>
            <span className='d-xxl-none'>$72,245</span>
            <span className='d-none d-xxl-inline-block'>$171,473</span>
            <span className='trending trending-down-basic'>
              <span>%12</span>
              <i className='os-icon os-icon-arrow-2-down'></i>
            </span>
          </div>
        </div>
        <div className='balance-table pl-sm-2'>
          <table className='table table-lightborder table-bordered table-v-compact mb-0'>
            <tbody>
              <tr>
                <td>
                  <strong>2.34</strong>
                  <div className='balance-label smaller lighter text-nowrap'>
                    Bitcoins BTC
                  </div>
                </td>
                <td>
                  <strong>1.74</strong>
                  <div className='balance-label smaller lighter text-nowrap'>
                    Ripple RPX
                  </div>
                </td>
                <td className='d-sm-none d-xxxxl-table-cell d-md-table-cell d-xxl-none'>
                  <strong>4.82</strong>
                  <div className='balance-label smaller lighter text-nowrap'>
                    Newcoin NCN
                  </div>
                </td>
              </tr>
              <tr>
                <td>
                  <strong>1.22</strong>
                  <div className='balance-label smaller lighter text-nowrap'>
                    Litecoin LTC
                  </div>
                </td>
                <td>
                  <strong>1.87</strong>
                  <div className='balance-label smaller lighter text-nowrap'>
                    Etherium ETH
                  </div>
                </td>
                <td className='d-sm-none d-xxxxl-table-cell d-md-table-cell d-xxl-none'>
                  <strong>1.02</strong>
                  <div className='balance-label smaller lighter text-nowrap'>
                    Dogecoin DGC
                  </div>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
      <div className='element-wrapper pb-4 mb-4 border-bottom'>
        <div className='element-box-tp'>
          <Link className='btn btn-primary' href={RoutePath.Properties}>
            <i className='os-icon os-icon-plus-circle'></i>
            <span>Buy Property</span>
          </Link>
        </div>
      </div>
    </div>
  );
}
