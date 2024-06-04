import { Metadata } from 'next';
import Dashboard from '.';

export const metadata: Metadata = {
  title: 'xHousing | Dashboard',
  description: 'Managed your xProject investments'
};

export default function DashboardPage() {
  return <Dashboard />;
}
