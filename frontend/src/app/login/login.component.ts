import { HttpClient } from '@angular/common/http';
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { AuthService } from '../services/auth.service';
import { Router } from '@angular/router';

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

  constructor(private http: HttpClient, private authService: AuthService, private router: Router) {
    
  }

  login() {
    this.http.post("/api/login", {
      email: this.email,
      password: this.password
    }).subscribe(data => {
      this.authService.retrieveUser();
      this.router.navigate(['/account']);
    })
  }
}
