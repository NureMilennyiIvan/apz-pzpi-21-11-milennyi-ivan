export interface AuthService<Client> {
    authorize(username: string, passwordHash: string): Promise<Client | null>;
}