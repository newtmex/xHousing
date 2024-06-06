import { Metadata } from 'next';
import Properties from '.';

export const metadata: Metadata = {
  title: 'xHousing | Properties',
  description: 'Prperties for Sale or rent'
};

export default function PropertiesPage() {
  return <Properties />
}
