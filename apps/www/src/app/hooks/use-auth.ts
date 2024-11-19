import { useCallback, useEffect, useState } from 'react'
import { User } from '../types/user'

type ErrorValidation = {
  status: boolean
  message: string
}

async function refreshToken() {
  const res = await fetch('/api/v1/auth/refresh-token', {
    method: 'GET',
    credentials: 'include',
  })
  if (!res.ok) {
    throw new Error('Failed to refresh token')
  }
  return true
}

export function useAuth() {
  const [user, setUser] = useState<User | null>(null)
  const [loading, setLoading] = useState(true)

  const fetchUser = useCallback(() => {
    fetch('/api/v1/user/profile', {
      method: 'GET',
      credentials: 'include',
    })
      .then((res) => res.json() as Promise<User>)
      .then((user) => setUser(user))
      .catch(() => {
        const refresh = async () => {
          try {
            await refreshToken()
            fetchUser()
            setLoading(false)
          } catch (error) {
            setUser(null)
            setLoading(false)
            console.error(error)
          }
        }
        refresh()
      })
  }, [])

  useEffect(() => {
    setLoading(true)
    fetch('/api/v1/user/profile', {
      method: 'GET',
      credentials: 'include',
    })
      .then((res) => res.json() as Promise<User>)
      .then((user) => {
        setUser(user)
        setLoading(false)
      })
      .catch(() => {
        setUser(null)
        setLoading(false)
      })
  }, [])

  const signIn = useCallback(async (username: string, password: string) => {
    fetch('/api/v1/auth/signin', {
      method: 'POST',
      credentials: 'include',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ username, password }),
    })
      .then((res) => {
        if (res.ok) {
          return true
        } else {
          throw new Error('Failed to sign in')
        }
      })
      .catch(() => {
        throw new Error('Invalid username or password')
      })
  }, [])

  const validatePassword = (password: string): ErrorValidation => {
    if (password.length < 8) {
      return {
        status: false,
        message: 'Password must be at least 8 characters',
      }
    }

    return {
      status: true,
      message: '',
    }
  }

  const validateUsername = (username: string): ErrorValidation => {
    if (username.length < 3) {
      return {
        status: false,
        message: 'Username must be at least 3 characters',
      }
    }

    return {
      status: true,
      message: '',
    }
  }

  return {
    user,
    loading,
    signIn,
    validatePassword,
    validateUsername,
  }
}
