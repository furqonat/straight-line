import React from 'react'
import { useAuth } from '../hooks/use-auth'

const UseAuth = React.createContext<ReturnType<typeof useAuth> | null>(null)

type AuthProviderProps = {
  children: React.ReactNode
}

export function AuthProvider({ children }: AuthProviderProps) {
  return <UseAuth.Provider value={useAuth()}>{children}</UseAuth.Provider>
}
