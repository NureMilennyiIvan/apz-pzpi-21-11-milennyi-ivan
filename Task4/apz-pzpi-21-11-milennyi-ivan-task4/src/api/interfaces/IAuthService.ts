export interface AuthService<Client> {
    checkUsername(user: Client): Promise<boolean>;
    authorize(username: string, passwordHash: string): Promise<Client | null>;
}