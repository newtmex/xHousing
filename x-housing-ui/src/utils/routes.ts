export enum RoutePath {
  Dashboard = '/dashboard',
  Properties = '/properties',
  Unlock = '/unlock'
}
export const ROUTES: { name: string; path: string; osIcon: string }[] = [
  { name: 'Dashboard', osIcon: 'layout', path: RoutePath.Dashboard },
  { name: 'Properties', osIcon: 'layers', path: RoutePath.Properties }
];
