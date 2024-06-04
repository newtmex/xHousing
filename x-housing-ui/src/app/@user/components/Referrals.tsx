export default function Referrals() {
  return (
    <div className='element-wrapper compact pt-4'>
      <div className='element-actions d-none d-sm-block'>
        <a className='btn btn-primary btn-sm' href='#'>
          <i className='os-icon os-icon-ui-22'></i>
          <span>Add Account</span>
        </a>
        <a className='btn btn-success btn-sm' href='#'>
          <i className='os-icon os-icon-grid-10'></i>
          <span>Make Payment</span>
        </a>
      </div>
      <h6 className='element-header'>Send Money To</h6>
      <div className='element-box-tp'>
        <div className='inline-profile-tiles'>
          <div className='row'>
            <div className='col-4 col-sm-3 col-xxl-2'>
              <div className='profile-tile profile-tile-inlined'>
                <a
                  className='profile-tile-box faded'
                  href='users_profile_small.html'
                >
                  <div className='pt-new-icon'>
                    <i className='os-icon os-icon-plus'></i>
                  </div>
                  <div className='pt-user-name'>
                    New
                    <br /> Account
                  </div>
                </a>
              </div>
            </div>
            <div className='col-4 col-sm-3 col-xxl-2'>
              <div className='profile-tile profile-tile-inlined'>
                <a className='profile-tile-box' href='users_profile_small.html'>
                  <div className='pt-avatar-w'>
                    <img alt='' src='img/avatar1.jpg' />
                  </div>
                  <div className='pt-user-name'>
                    Kelly
                    <br /> Neymayers
                  </div>
                </a>
              </div>
            </div>
            <div className='col-4 col-sm-3 col-xxl-2'>
              <div className='profile-tile profile-tile-inlined'>
                <a className='profile-tile-box' href='users_profile_small.html'>
                  <div className='pt-avatar-w'>
                    <img alt='' src='img/avatar3.jpg' />
                  </div>
                  <div className='pt-user-name'>
                    Ben
                    <br /> Gossman
                  </div>
                </a>
              </div>
            </div>
            <div className='col-4 col-sm-3 col-xxl-2'>
              <div className='profile-tile profile-tile-inlined'>
                <a className='profile-tile-box' href='users_profile_small.html'>
                  <div className='pt-avatar-w'>
                    <img alt='' src='img/avatar1.jpg' />
                  </div>
                  <div className='pt-user-name'>
                    Kimson
                    <br /> Broker
                  </div>
                </a>
              </div>
            </div>
            <div className='col-4 d-sm-none d-xxl-block col-xxl-2'>
              <div className='profile-tile profile-tile-inlined'>
                <a className='profile-tile-box' href='users_profile_small.html'>
                  <div className='pt-avatar-w'>
                    <img alt='' src='img/avatar2.jpg' />
                  </div>
                  <div className='pt-user-name'>
                    Jake
                    <br /> Gilbertson
                  </div>
                </a>
              </div>
            </div>
            <div className='col-4 d-sm-none d-xxl-block col-xxl-2'>
              <div className='profile-tile profile-tile-inlined'>
                <a className='profile-tile-box' href='users_profile_small.html'>
                  <div className='pt-avatar-w'>
                    <img alt='' src='img/avatar7.jpg' />
                  </div>
                  <div className='pt-user-name'>
                    Mary
                    <br /> Clintons
                  </div>
                </a>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
