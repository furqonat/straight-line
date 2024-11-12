import { render } from '@testing-library/react'

import { AppPage } from './app-page'
import { BrowserRouter } from 'react-router-dom'

describe('App-Page', () => {
  it('should render successfully', () => {
    const { baseElement } = render(
      <BrowserRouter>
        <AppPage />
      </BrowserRouter>
    )
    expect(baseElement).toBeTruthy()
  })

  it('should have a greeting as the title', () => {
    const { getByText, getByAltText } = render(
      <BrowserRouter>
        <AppPage />
      </BrowserRouter>
    )
    expect(
      getByText(/Straight Line just messaging without any data collection./gi)
    ).toBeTruthy()
    expect(getByText(/Get Started/gi)).toBeTruthy()
    expect(getByAltText(/messaging/gi)).toBeTruthy()
    expect(getByText(/Sign In/gi)).toBeTruthy()
    expect(getByText(/About/gi)).toBeTruthy()
  })
})
