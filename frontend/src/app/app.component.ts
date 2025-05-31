import { HttpClient } from '@angular/common/http';
import { Component } from '@angular/core';
import { Router, RouterLink, RouterLinkActive, RouterOutlet } from '@angular/router';
import { AuthService } from './services/auth.service';
import { Observable } from 'rxjs';
import { User } from './models/user.model';
import { CommonModule } from '@angular/common';

interface TestResponse {
  test: number;
  title: string;
}


@Component({
  selector: 'app-root',
  imports: [RouterLink, RouterLinkActive, RouterOutlet, CommonModule],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css'
})
export class AppComponent {
  title = '';
  user: Observable<User | null>;

  constructor(private http: HttpClient, private authService: AuthService) {
    this.http.get<TestResponse>('/api/test').subscribe(data => {
      console.log('data', data);
      this.title = data.title;
    });

    this.user = this.authService.user;
  }

  logout(event: MouseEvent) {
    event.preventDefault();
    this.authService.logout();
  }
}
