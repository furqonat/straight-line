import { Route, Routes } from 'react-router-dom'
import { AppPage, SignInPage, SignUpPage } from './pages'

export function App() {
  return (
    <Routes>
      <Route path={'/'} element={<AppPage />} />
      <Route path={'/signin'} element={<SignInPage />} />
      <Route path={'/signup'} element={<SignUpPage />} />
    </Routes>
  )
}

export default App
