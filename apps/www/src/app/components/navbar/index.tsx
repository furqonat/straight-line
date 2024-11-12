import { ReactNode } from 'react'
import { Link } from 'react-router-dom'

export type MenuProps = {
  title: ReactNode | string
  href: string
}
type NavbarProps = {
  title: string
  menu?: MenuProps[]
}
export function Navbar(props: NavbarProps) {
  const { title, menu } = props
  return (
    <nav className={'flex flex-row gap-2 w-full border-b border-gray-200 p-5'}>
      <div className={'container mx-auto flex gap-2'}>
        <div className={'flex-1 flex flex-col justify-center'}>
          <a className={'font-semibold text-lg'} href={'/'}>
            {title}
          </a>
        </div>
        <ul className={'menu menu-horizontal'}>
          {menu?.map((item) => (
            <li className={'justify-center'}>
              <Link to={item.href}>{item.title}</Link>
            </li>
          ))}
        </ul>
      </div>
    </nav>
  )
}
