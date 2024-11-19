import { useState } from 'react'
import { EyeSlash, EyeVisible, Navbar } from '../../../components'
import { useAuth } from '../../../hooks'

export function SignInPage() {
  const { signIn } = useAuth()
  const [username, setUsername] = useState('')
  const [password, setPassword] = useState('')
  const [passwordVisible, setPasswordVisible] = useState(false)
  const [passwordInputType, setPasswordInputType] = useState('password')

  function togglePasswordVisibility() {
    setPasswordVisible(!passwordVisible)
    setPasswordInputType(passwordVisible ? 'password' : 'text')
  }

  function handleChangeUsername(e: React.ChangeEvent<HTMLInputElement>) {
    setUsername(e.target.value)
  }

  function handleChangePassword(e: React.ChangeEvent<HTMLInputElement>) {
    setPassword(e.target.value)
  }

  function handleOnSignIn(ev: React.MouseEvent<HTMLButtonElement>) {
    ev.preventDefault()
    signIn(username, password)
      .then(() => {
        // do something
      })
      .catch((error) => {
        console.error(error)
      })
  }

  return (
    <main>
      <Navbar title={'Straight Line'} />
      <section className={'container mx-auto'}>
        <label className={'form-control w-full max-w-xs'}>
          <div className={'label'}>
            <span className={'label-text'}>Username</span>
          </div>
          <input
            value={username}
            onChange={handleChangeUsername}
            type="text"
            placeholder="username"
            className={
              'input input-bordered w-full max-w-xs input-sm md:input-md'
            }
          />
        </label>
        <label className={'form-control w-full max-w-xs'}>
          <div className={'label'}>
            <span className={'label-text'}>Password</span>
          </div>
          <div className={'join'}>
            <input
              value={password}
              onChange={handleChangePassword}
              type={passwordInputType}
              placeholder="password"
              className={
                'join-item input input-bordered w-full max-w-xs input-sm md:input-md'
              }
            />
            <button
              className={'btn  join-item'}
              onClick={togglePasswordVisibility}
            >
              {passwordVisible ? (
                <EyeVisible size={6} />
              ) : (
                <EyeSlash size={6} />
              )}
            </button>
          </div>
        </label>
        <button
          onClick={handleOnSignIn}
          className={'btn btn-primary w-full max-w-xs mt-4'}
        >
          Sign In
        </button>
      </section>
    </main>
  )
}
