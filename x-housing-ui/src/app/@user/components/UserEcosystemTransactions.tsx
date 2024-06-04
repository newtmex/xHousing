export default function UserEcosystemTransactions() {
  return (
    <div className='element-wrapper compact pt-4'>
      <div className='element-actions'>
        <form className='form-inline justify-content-sm-end'>
          <label className='smaller'>Order By</label>
          <select className='form-control form-control-sm form-control-faded'>
            <option value='Pending'>Today</option>
            <option value='Active'>Last Week</option>
            <option value='Cancelled'>Last 30 Days</option>
          </select>
        </form>
      </div>
      <h6 className='element-header'>Transactions</h6>
      <div className='element-box-tp'>
        <table className='table table-clean'>
          <tbody>
            <tr>
              <td>
                <div className='value'>Amazon Store</div>
                <span className='sub-value'>Books</span>
              </td>
              <td className='text-right'>
                <div className='value'>-$28.34</div>
                <span className='sub-value'>12 Feb 2018</span>
              </td>
            </tr>
            <tr>
              <td>
                <div className='value'>Dunkin Donuts</div>
                <span className='sub-value'>Food &amp; Restaurants</span>
              </td>
              <td className='text-right'>
                <div className='value'>-$7.15</div>
                <span className='sub-value'>10 Feb 2018</span>
              </td>
            </tr>
            <tr>
              <td>
                <div className='value'>Refund from Sephora</div>
                <span className='sub-value'>Health &amp; Beauty</span>
              </td>
              <td className='text-right'>
                <div className='value text-success'>$128.11</div>
                <span className='sub-value'>10 Feb 2018</span>
              </td>
            </tr>
            <tr>
              <td>
                <div className='value'>Amazon Store</div>
                <span className='sub-value'>Books</span>
              </td>
              <td className='text-right'>
                <div className='value'>-$28.34</div>
                <span className='sub-value'>12 Feb 2018</span>
              </td>
            </tr>
            <tr>
              <td>
                <div className='value'>Dunkin Donuts</div>
                <span className='sub-value'>Food &amp; Restaurants</span>
              </td>
              <td className='text-right'>
                <div className='value'>-$7.15</div>
                <span className='sub-value'>10 Feb 2018</span>
              </td>
            </tr>
            <tr>
              <td>
                <div className='value'>Refund from Google Store</div>
                <span className='sub-value'>Health &amp; Beauty</span>
              </td>
              <td className='text-right'>
                <div className='value text-success'>$15.23</div>
                <span className='sub-value'>9 Feb 2018</span>
              </td>
            </tr>
            <tr>
              <td>
                <div className='value'>Amazon Store</div>
                <span className='sub-value'>Books</span>
              </td>
              <td className='text-right'>
                <div className='value'>-$28.34</div>
                <span className='sub-value'>8 Feb 2018</span>
              </td>
            </tr>
            <tr>
              <td>
                <div className='value'>Dunkin Donuts</div>
                <span className='sub-value'>Food &amp; Restaurants</span>
              </td>
              <td className='text-right'>
                <div className='value'>-$7.15</div>
                <span className='sub-value'>8 Feb 2018</span>
              </td>
            </tr>
          </tbody>
        </table>
        <a className='centered-load-more-link' href='#'>
          <span>Load More Messages</span>
        </a>
      </div>
    </div>
  );
}
