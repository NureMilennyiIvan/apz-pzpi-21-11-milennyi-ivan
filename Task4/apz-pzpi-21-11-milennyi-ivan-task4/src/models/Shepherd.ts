export class Shepherd {
    id: number | null;
    username: string;
    password: string;
    name: string;
    surname: string;

    constructor(id: number | null, username: string, password: string, name: string, surname: string) {
        this.id = id;
        this.username = username;
        this.password = password;
        this.name = name;
        this.surname = surname;
    }
}