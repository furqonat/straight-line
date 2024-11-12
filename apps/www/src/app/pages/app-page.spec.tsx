import { render } from '@testing-library/react'

import { AppPage } from './app-page'

describe('App-Page', () => {
  it('should render successfully', () => {
    const { baseElement } = render(<AppPage />)
    expect(baseElement).toBeTruthy()
  })

  it('should have a greeting as the title', () => {
    const { getByText } = render(<AppPage />)
    expect(getByText(/App Page/gi)).toBeTruthy()
  })
})
