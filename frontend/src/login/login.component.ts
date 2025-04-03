import { HttpClient } from '@angular/common/http';
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';

@Component({
  selector: 'app-login',
  imports: [
    FormsModule
  ],
  templateUrl: './login.component.html',
  styleUrl: './login.component.css'
})
export class LoginComponent {

  email = '';
  password = '';

  constructor(private http: HttpClient) {
  }

  login() {
    console.log('Hello: ' + this.email + ', ' + this.password);
    this.http.post("/api/login", {
      username: this.email,
      password: this.password
    }).subscribe(data => {
      console.log(data);
    })
  }
}
