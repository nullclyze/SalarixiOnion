class Particles {
  private canvas: HTMLCanvasElement | null = null;
  private ctx: CanvasRenderingContext2D | null = null;
  private particles: Array<any> = [];
  private enabled: boolean = false;

  constructor() {
    this.canvas = document.getElementById('particle-background') as HTMLCanvasElement;

    if (this.canvas) {
      this.ctx = this.canvas.getContext('2d');
      this.particles = [];
      this.enabled = false;

      const parent = this.canvas.parentElement;

      if (parent) {
        this.canvas.width = parent.clientWidth;
        this.canvas.height = parent.clientHeight;
      }

      this.spawn();
    }
  }

  private spawn() {
    this.particles = [];

    for (let i = 0; i < 80; i++) {
      const particle = {
        x: Math.random() * this.canvas!.width,
        y: Math.random() * this.canvas!.height,
        vx: (Math.random() - 0.5) * 1,
        vy: (Math.random() - 0.5) * 1,
        radius: 2
      };
      
      this.particles.push(particle);
    }
  }

  private update() {
    this.particles.forEach(p => {
      p.x += p.vx;
      p.y += p.vy;

      if (p.x < 0 || p.x > this.canvas!.width) {
        p.vx *= -1;
      }

      if (p.y < 0 || p.y > this.canvas!.height) {
        p.vy *= -1;
      }
    });
  }

  private draw() {
    this.ctx!.clearRect(0, 0, this.canvas!.width, this.canvas!.height);

    this.particles.forEach(p => {
      this.ctx!.beginPath();
      this.ctx!.arc(p.x, p.y, p.radius, 0, Math.PI * 2);
      this.ctx!.fillStyle = 'rgba(216, 216, 216, 0.6)'; 
      this.ctx!.fill();
      this.ctx!.closePath();
    });

    this.ctx!.setLineDash([]); 

    for (let i = 0; i < this.particles.length; i++) {
      for (let j = i + 1; j < this.particles.length; j++) {
        const p1 = this.particles[i];
        const p2 = this.particles[j];
        const distance = Math.sqrt((p1.x - p2.x) ** 2 + (p1.y - p2.y) ** 2);

        if (distance < 120) {
          this.ctx!.beginPath();
          this.ctx!.moveTo(p1.x, p1.y);
          this.ctx!.lineTo(p2.x, p2.y);
          this.ctx!.strokeStyle = 'rgba(182, 182, 182, 0.4)';
          this.ctx!.lineWidth = 1 - distance / 120; 
          this.ctx!.stroke();
          this.ctx!.closePath();
        }
      }
    }

    this.ctx!.setLineDash([]);
  }

  private animate() {
    if (!this.enabled) return;
    
    this.update();
    this.draw();

    requestAnimationFrame(() => this.animate());
  }

  public enable() {
    if (this.enabled) return;

    this.enabled = true;

    this.ctx!.reset();
    this.animate();
  }

  public disable() {
    this.ctx!.reset();
    this.enabled = false;
  }
}

const particles = new Particles();

export function enableParticles() {
  (document.getElementById('particle-wrapper') as HTMLElement).style.display = 'flex';
  particles.enable();
}

export function disableParticles() {
  (document.getElementById('particle-wrapper') as HTMLElement).style.display = 'none';
  particles.disable();
}