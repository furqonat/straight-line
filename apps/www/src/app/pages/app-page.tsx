import { Navbar } from '../components'
import messaging from '../../assets/messaging.svg'
import { Link } from 'react-router-dom'

export function AppPage() {
  return (
    <>
      <Navbar
        title={'Straight Line'}
        menu={[
          {
            title: (
              <button className={'btn btn-secondary btn-sm'}>Sign In</button>
            ),
            href: '/signin',
          },
          {
            title: 'About',
            href: '/about',
          },
        ]}
      />
      <main className={'container mx-auto'}>
        <section
          className={
            'grid grid-cols-1 md:grid-cols-2 gap-4 min-h-[80vh] justify-center items-center'
          }
        >
          <section className={'flex flex-col gap-4'}>
            <h1 className={'text-4xl font-bold'}>Straight Line</h1>
            <p className={'text-lg'}>
              Straight Line just messaging without any data collection.
            </p>
            <div>
              <Link
                to={'/signin'}
                className={'btn btn-primary md:w-auto text-white font-semibold'}
              >
                Get Started
              </Link>
            </div>
          </section>
          <section
            className={'w-full flex flex-col gap-4 justify-center items-center'}
          >
            <img
              src={messaging}
              alt={'messaging'}
              className={'w-full md:w-3/4'}
            />
          </section>
        </section>
        <section className={'container mx-auto'}>{/* TODO */}</section>
      </main>
    </>
  )
}
