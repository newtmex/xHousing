import ElementWrapper from './ElementWrapper';

export default function EcosystemTransactions() {
  if (true) {
    return null;
  }
  // TODO
  return (
    <ElementWrapper title='Ecosytem Transactions'>
      <table className='table table-compact smaller text-faded mb-0'>
        <thead>
          <tr>
            <th>Type</th>
            <th className='text-center'>Date</th>
            <th className='text-right'>Amount</th>
            <th className='text-right'>Fee</th>
          </tr>
        </thead>
        <tbody>
          <tr>
            <td>
              <span>BTC</span>
              <i className='os-icon os-icon-repeat icon-separator'></i>
              <span>USD</span>
            </td>
            <td className='text-center'>01.08</td>
            <td className='text-right text-bright'>$25.38</td>
            <td className='text-right text-danger'>-$1.23</td>
          </tr>
          <tr>
            <td>
              <span>RPX</span>
              <i className='os-icon os-icon-repeat icon-separator'></i>
              <span>ETH</span>
            </td>
            <td className='text-center'>01.07</td>
            <td className='text-right text-bright'>$15.21</td>
            <td className='text-right text-danger'>-$1.13</td>
          </tr>
          <tr>
            <td>
              <span>LTC</span>
              <i className='os-icon os-icon-repeat icon-separator'></i>
              <span>BTC</span>
            </td>
            <td className='text-center'>01.05</td>
            <td className='text-right text-bright'>$17.43</td>
            <td className='text-right text-danger'>-$2.14</td>
          </tr>
          <tr>
            <td>
              <span>PRX</span>
              <i className='os-icon os-icon-repeat icon-separator'></i>
              <span>LTC</span>
            </td>
            <td className='text-center'>01.05</td>
            <td className='text-right text-bright'>$23.18</td>
            <td className='text-right text-danger'>-$3.17</td>
          </tr>
          <tr>
            <td>
              <span>ETH</span>
              <i className='os-icon os-icon-repeat icon-separator'></i>
              <span>USD</span>
            </td>
            <td className='text-center'>01.04</td>
            <td className='text-right text-bright'>$35.42</td>
            <td className='text-right text-danger'>-$3.12</td>
          </tr>
          <tr>
            <td>
              <span>BTC</span>
              <i className='os-icon os-icon-repeat icon-separator'></i>
              <span>ETH</span>
            </td>
            <td className='text-center'>01.02</td>
            <td className='text-right text-bright'>$72.62</td>
            <td className='text-right text-danger'>-$4.15</td>
          </tr>
          <tr>
            <td>
              <span>RPX</span>
              <i className='os-icon os-icon-repeat icon-separator'></i>
              <span>USD</span>
            </td>
            <td className='text-center'>12.29</td>
            <td className='text-right text-bright'>$25.38</td>
            <td className='text-right text-danger'>-$1.23</td>
          </tr>
          <tr>
            <td>
              <span>ETH</span>
              <i className='os-icon os-icon-repeat icon-separator'></i>
              <span>BTC</span>
            </td>
            <td className='text-center'>12.28</td>
            <td className='text-right text-bright'>$52.11</td>
            <td className='text-right text-danger'>-$4.72</td>
          </tr>
        </tbody>
      </table>
    </ElementWrapper>
  );
}
