import { ROUTES } from '@/utils/routes';
import UserAvatar from './UserAvatar';
import UserInfo from './UserInfo';

export default function MobileMenu() {
  return (
    <div className='menu-mobile menu-activated-on-click color-scheme-dark'>
      <div className='mm-logo-buttons-w'>
        <a className='mm-logo' href='/'>
          <img src='img/logo.png' />
          <span>xHousing</span>
        </a>
        <div className='mm-buttons'>
          <div className='content-panel-open'>
            <div className='os-icon os-icon-grid-circles'></div>
          </div>
          <div className='mobile-menu-trigger'>
            <div className='os-icon os-icon-hamburger-menu-1'></div>
          </div>
        </div>
      </div>
      <div className='menu-and-user'>
        <div className='logged-user-w'>
          <UserAvatar />
          <UserInfo />
        </div>

        <ul className='main-menu'>
          {ROUTES.map((route, index) => (
            <li key={`${route.path}+${index}`}>
              <a href={route.path}>
                <div className='icon-w'>
                  <div className={`os-icon os-icon-${route.osIcon}`}></div>
                </div>
                <span>{route.name}</span>
              </a>
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
}
