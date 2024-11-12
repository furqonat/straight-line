import { Route, Routes } from 'react-router-dom'
import { AppPage } from './pages'

export function App() {
  return (
    <Routes>
      <Route path={'/'} element={<AppPage />} />
    </Routes>
  )
}

export default App
