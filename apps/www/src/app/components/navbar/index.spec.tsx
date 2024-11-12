import { render } from '@testing-library/react'
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
})
