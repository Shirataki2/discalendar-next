export const useNavbar = () => {
  const navBar = useState<boolean | null>('navbar', () => null);
  const setNavbar = (value: boolean) => navBar.value = value;
  const toggleNavbar = () => navBar.value = !navBar.value;

  return { navBar, setNavbar, toggleNavbar }
}
