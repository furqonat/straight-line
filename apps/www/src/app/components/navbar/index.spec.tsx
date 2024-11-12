import { render, screen } from '@testing-library/react'
import { BrowserRouter } from 'react-router-dom'
import { Navbar } from './index'
describe('Navbar', () => {
  it('should render successfully', () => {
    const { baseElement } = render(<Navbar title={'test'} />)
    expect(baseElement).toBeTruthy()
  })

  it('test title', () => {
    const { getByText } = render(<Navbar title={'test'} />)
    expect(getByText(/test/gi)).toBeTruthy()
  })

  it('test menu', () => {
    render(
      <BrowserRouter>
        <Navbar
          title={'test'}
          menu={[
            {
              title: 'menu',
              href: '/menu',
            },
          ]}
        />
      </BrowserRouter>
    )
    expect(screen.getByText(/menu/gi)).toBeTruthy()
  })
})
