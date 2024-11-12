import { useCallback, useEffect, useState } from 'react'
import { User } from '../types/user'

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
