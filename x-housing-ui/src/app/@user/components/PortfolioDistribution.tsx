export default function PortfolioDistribution() {
  return (
    <div className='col-sm-2 d-none d-xxl-block'>
      <div className='element-box less-padding'>
        <h6 className='element-box-header text-center'>
          Portfolio Distribution
        </h6>
        <div className='el-chart-w'>
          <div className='chartjs-size-monitor'>
            <div className='chartjs-size-monitor-expand'>
              <div className=''></div>
            </div>
            <div className='chartjs-size-monitor-shrink'>
              <div className=''></div>
            </div>
          </div>
          <canvas
            height='342'
            id='donutChart1'
            width='342'
            className='chartjs-render-monitor'
          ></canvas>
          <div className='inside-donut-chart-label'>
            <strong>263</strong>
            <span>Coins</span>
          </div>
        </div>
        <div className='el-legend condensed'>
          <div className='row'>
            <div className='col-auto col-xxxxl-6 ml-sm-auto mr-sm-auto'>
              <div className='legend-value-w'>
                <div className='legend-pin legend-pin-squared'></div>
                <div className='legend-value'>
                  <span>Bitcoins</span>
                  <div className='legend-sub-value'>17%, 12 Coins</div>
                </div>
              </div>
              <div className='legend-value-w'>
                <div className='legend-pin legend-pin-squared'></div>
                <div className='legend-value'>
                  <span>Litecoin</span>
                  <div className='legend-sub-value'>22%, 0.3 Coins</div>
                </div>
              </div>
            </div>
            <div className='col-sm-6 d-none d-xxxxl-block'>
              <div className='legend-value-w'>
                <div className='legend-pin legend-pin-squared'></div>
                <div className='legend-value'>
                  <span>Etherium</span>
                  <div className='legend-sub-value'>3%, 7 Coins</div>
                </div>
              </div>
              <div className='legend-value-w'>
                <div className='legend-pin legend-pin-squared'></div>
                <div className='legend-value'>
                  <span>Ripple</span>
                  <div className='legend-sub-value'>15%, 4 Coins</div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
