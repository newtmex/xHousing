export default function PortfolioValue() {
  return (
    <div className='fancy-selector-w'>
      <div className='fancy-selector-current'>
        <div className='fs-img'>
          <img alt='' src='img/card4.png' />
        </div>
        <div className='fs-main-info'>
          <div className='fs-name'>
            <span>Bitcoin Portfolio</span>
            <strong>BTC</strong>
          </div>
          <div className='fs-sub'>
            <span>Balance:</span>
            <strong>$5,304</strong>
          </div>
        </div>
        <div className='fs-selector-trigger'>
          <i className='os-icon os-icon-arrow-down4'></i>
        </div>
      </div>
      <div className='fancy-selector-options'>
        <div className='fancy-selector-option'>
          <div className='fs-img'>
            <img alt='' src='img/card2.png' />
          </div>
          <div className='fs-main-info'>
            <div className='fs-name'>
              <span>Lite Portfolio</span>
              <strong>ETH</strong>
            </div>
            <div className='fs-sub'>
              <span>Balance:</span>
              <strong>$5,304</strong>
            </div>
          </div>
        </div>
        <div className='fancy-selector-option active'>
          <div className='fs-img'>
            <img alt='' src='img/card4.png' />
          </div>
          <div className='fs-main-info'>
            <div className='fs-name'>
              <span>Bitcoin Portfolio</span>
              <strong>BTC</strong>
            </div>
            <div className='fs-sub'>
              <span>Balance:</span>
              <strong>$8,274</strong>
            </div>
          </div>
        </div>
        <div className='fancy-selector-option'>
          <div className='fs-img'>
            <img alt='' src='img/card3.png' />
          </div>
          <div className='fs-main-info'>
            <div className='fs-name'>
              <span>Ripple Portfolio</span>
              <strong>RPX</strong>
            </div>
            <div className='fs-sub'>
              <span>Balance:</span>
              <strong>$1,202</strong>
            </div>
          </div>
        </div>
        <div className='fancy-selector-actions text-right'>
          <a className='btn btn-primary' href='#'>
            <i className='os-icon os-icon-ui-22'></i>
            <span>Add Account</span>
          </a>
        </div>
      </div>
    </div>
  );
}
