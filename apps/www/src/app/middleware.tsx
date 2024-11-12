import { Navigate, Outlet, useLocation } from 'react-router-dom'
import { useAuth } from './hooks/use-auth'

const allowed_paths = ['/signin', '/signup', '/']

export function Middleware() {
  const { loading, user } = useAuth()
  const location = useLocation()

  if (loading) {
    return (
      <main
        className={
          'container mx-auto min-h-screen justify-center items-center w-full'
        }
      >
        <span className={'loading loading-bars loading-lg'} />
      </main>
    )
  }

  return user && allowed_paths.includes(location.pathname) ? (
    <Outlet />
  ) : (
    <Navigate to={'/signin'} />
  )
}
