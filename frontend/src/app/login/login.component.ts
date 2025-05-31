import { HttpClient } from '@angular/common/http';
import { Component } from '@angular/core';
import { FormsModule } from '@angular/forms';
import { AuthService } from '../services/auth.service';
import { Router } from '@angular/router';
import { map } from 'rxjs';
import { CommonModule } from '@angular/common';


interface LoginResponse {
  mfaId: string;
  status: string,
}

@Component({
  selector: 'app-login',
  imports: [
    FormsModule,
    CommonModule,
  ],
  templateUrl: './login.component.html',
  styleUrl: './login.component.css'
})
export class LoginComponent {

  email = '';
  password = '';
  mfaCode = '';
  needsMfa = false;

  constructor(private http: HttpClient, private authService: AuthService, private router: Router) {
    
  }

  login() {
    this.http.post<LoginResponse>("/api/login", {
      email: this.email,
      password: this.password
    }).subscribe((data: LoginResponse) => {
      console.log('LoginResponse', data);
      if (data.status == "MfaNeeded") {
        console.log('MFA needed !!! Type', data.mfaId);
        this.needsMfa = true;
      } else {
        this.authService.retrieveUser();
        this.router.navigate(['/account']);
      }
    })
  }

  sendCode() {
    this.http.post("/api/login/mfa", {
      code: this.mfaCode
    }).subscribe(data => {
      console.log('Mfa response', data);
      this.authService.retrieveUser();
      this.router.navigate(['/account']);
    });
  }

}
