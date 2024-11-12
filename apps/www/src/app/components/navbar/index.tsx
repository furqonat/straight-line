type NavbarProps = {
  title: string
}
export function Navbar(props: NavbarProps) {
  const { title } = props
  return (
    <nav className={'flex flex-row gap-2 w-full border-b border-gray-300 p-5'}>
      <div className={'container mx-auto'}>
        <div className={'flex-1'}>
          <a className={'font-semibold text-lg'} href={'/'}>
            {title}
          </a>
        </div>
        <div>{/* TODO: add navigation */}</div>
      </div>
    </nav>
  )
}
