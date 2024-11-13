import { useCallback, useEffect, useState } from 'react'
import { User } from '../types/user'

const apiUrl = import.meta.env.VITE_API_URL

export function useAuth() {
  const [user, setUser] = useState<User | null>(null)
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    // TODO
  }, [])

  const signIn = useCallback(() => {
    // TODO
  }, [])

  return {
    user,
    loading,
    signIn,
  }
}
