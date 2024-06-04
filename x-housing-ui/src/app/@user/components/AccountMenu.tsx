import { getWindowLocation, logout } from '@/utils';
import { useRouter } from 'next/navigation';

export default function AccountMenu() {
  const router = useRouter();

  const onRedirect = () => {
    router.replace('/unlock');
  };

  const handleLogout = () => {
    const { href } = getWindowLocation();
    sessionStorage.clear();
    logout(href, onRedirect, false);
  };
  return (
    <ul>
      <li>
        <button className='btn btn-danger' onClick={handleLogout}>
          <i className='os-icon os-icon-signs-11'></i>
          <span>Logout</span>
        </button>
      </li>
    </ul>
  );
}
